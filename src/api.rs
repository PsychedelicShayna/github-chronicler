/*
*                  SOME API CLARIFICATION TO AVOID FUTURE CONFUSION
*
*                       (GitHub's API can be a little tricky)
*                          (sly, sly Octopus- I mean, cat)
*
* The API operates on UTC+0, with a new week starting on Monday. The relation
* between when the API endpoints update and how much time passes is not very
* obvious however.
*
* The easiest are the views and clones, as they have a daily (hourly, really)
* report endpoint, which comes with an exact timestamp for every hour, making
* it trivial to collect and store as it comes in. The timestamps become keys
* in a key:value store, and the total can easily be calculated: sum them all.
*
*     (although in practice, the last sum should be stored, O(1), or else
*       it's gonna be an O(n) disaster that'll just get worse over time)
*
* Likewise, the weekly views and clones, while a bit more troublesome, is also relatively easy.
* Weekly metrics come in the form of three timestamps, the difference between the first and the
* second being 7 days, and the difference between the third and the second being another 7 days.
*
* The third timestamp is prone to change, whereas the first and second are set in stone. This
* makes it easy to collect weekly information despite the data being bi-weekly (14 days), as
* the third timestamp can simply be ignored until the first timestamp changes, going past 15
* days, at which point the third becomes the second, the second becomes first, and there is
* another week of metrics ready to store.
*
* The problem lies with the referrals and popular content traffic, as they have no timestamp,
* and unlike the weekly metrics for views and clones, do not cover 7 day batches, instead they
* update daily. The only way to know how many referrals or traffic occurred on one day is to
* wait precisely before UTC midnight, make an API request as close to midnight as possible,
* and once midnight passes, immediately make another API request as soon as possible. As the
* 14th day becomes the 15th, its contribution to the overall count will be gone, the number
* being left lower than it was before. The reason we need to make requests before and after
* midnight as closely as possible, is due to the fact that, say, the total at a given time
* would be 20 referrals, then, the 14th day rolls over to the 15th, and the number drops by
* 5, down to 15, but then before another API request has been made to measure the drop, a
* few referrals come in, say 2, bumping 15 back up to 17. If we aren't precise and do the
* API request in an hour or so, that leaves time for the next API request to report 17,
* which is 3 less than 20, not 5, leading to a miscalculation that only 3 referrals occurred
* 15, formerly 14, days ago, when it was really 5, the 2 new views obscure the true result.
*
* I can't think of a better way than to ride along the tail of the timestamp given by the
* hourly reports, and when the hour reaches one before midnight, sleep until 2 minutes or
* so before the shift, and start performing API requests throughout, until a difference is
* observed, and assume that no significant amount of traffic could have occurred in such a
* small amount of time, and simply regard whatever the difference is as the traffic for what
* is now 15 days ago. If the new day rolls in, increasing the number, as the old one rolls
* out, decreasing the number, all within the same request, then things might get annoying.
* Then I can only think of correlating the views with the amount gained and lost, and some
* how deducing the best I can on what day the referrals and content visits happened. I'll
* have to figure something out...
*
* *-*-*-*-*-* Excerpt *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
*
*  https://docs.github.com/en/repositories/viewing-activity-and-data-for-your-repository/
*
*   Referring sites and popular content are ordered by views and unique
*   visitors. Full clones and visitor information update hourly, while
*   referring sites and popular content sections update daily. All data in
*   the traffic graph uses the UTC+0 timezone, regardless
*   of your location.
*
* *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
*/

use anyhow as ah;
use minreq::get;
use serde::de::DeserializeOwned;

use crate::api_models::*;

const API_BASE: &str = "https://api.github.com";

typedef!(pub, AuthToken, String);
typedef!(pub, EndpointURL, String);
typedef!(pub, EndpointTemplate, String);
typedef!(pub, URL, String);

fn attempt_api_request<T: DeserializeOwned>(token: &AuthToken, url: &String) -> ah::Result<T> {
    let request = get(url)
        .with_header("User-Agent", "PsychedelicShayna")
        .with_header("Accept", "application/vnd.github+json")
        .with_header("Authorization", format!("Bearer {}", token.0))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .with_timeout(2048);

    println!("Using token: {}", token.0);
    println!("Sending request {:?}", request);

    let response = request.send()?;

    println!("Received response {:?}", response);

    let status_code: &i32 = &response.status_code;

    std::thread::sleep(std::time::Duration::from_millis(128));

    match status_code {
        200 => {
            let content = response.as_str()?.to_string();
            let deserialized: T = sj::from_str::<T>(&content).map_err(|e| ah::anyhow!(e))?;
            Ok(deserialized)
        }
        code => Err(ah::anyhow!(
            "GET: {}\nResponse: Request failed with status code {}, {}, {:?}",
            url,
            code,
            response.reason_phrase,
            response.as_str()?
        )),
    }
}

macro_rules! define_request_fn {
    ($name:ident, $type:ty, $endpoint:expr) => {
        pub fn $name(token: &AuthToken, author: &String, repo: &String) -> ah::Result<$type> {
            let endpoint = format!($endpoint, API_BASE, author, repo);
            attempt_api_request(token, &endpoint)
        }
    };
}

define_request_fn!(
    request_clones_daily,
    ModelRepoClonesDaily,
    "{}/repos/{}/{}/traffic/clones?per=day"
);

define_request_fn!(
    request_clones_weekly,
    ModelRepoClonesBiWeekly,
    "{}/repos/{}/{}/traffic/clones?per=week"
);

define_request_fn!(
    request_views_daily,
    ModelRepoViewsDaily,
    "{}/repos/{}/{}/traffic/views?per=day"
);

define_request_fn!(
    request_views_weekly,
    ModelRepoViewsBiWeekly,
    "{}/repos/{}/{}/traffic/views?per=week"
);

define_request_fn!(
    request_referrers_weekly,
    ModelReferrerals,
    "{}/repos/{}/{}/traffic/popular/referrers"
);

define_request_fn!(
    request_popular_paths_weekly,
    ModelContentTrafficBiWeekly,
    "{}/repos/{}/{}/traffic/popular/paths"
);

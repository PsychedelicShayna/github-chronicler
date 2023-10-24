import requests, json, time, sys, os
from requests import Response
from datetime import datetime
from typing import Optional

def get_utc_timestamp() -> str:
    utc_time: datetime = datetime.utcnow()
    format: str = "%Y-%m-%dT%H:%M:%SZ"
    return utc_time.strftime(format)


def perform_request(request_url: str) -> requests.Response:
    response = requests.request(
        url=request_url,
        method="get",
        headers={
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {token}",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": owner,
        },
    )

    return response


def read_json(file: str) -> dict:
    with open(file, "r") as f:
        return json.load(f)

def write_json(file: str, data: dict, indent: Optional[int] = None):
    with open(file, "w+") as f:
        json.dump(data, f, indent = indent)

# A background API response collector for seeing how the data changes over
# time, notably what happens at UTC midnight. Can't wait to find out!
# I need confirmation before I can continue, lest I operate on faulty
# assumptions..

def debug_bg_collector(url: str, output: str, sleepy_time: int = 60):
    print("Starting collector...")
    
    if not os.path.isfile(output):
        with open(output, 'w+') as f:
            f.write(json.dumps({}, indent=4))

    while True:
        response: Response | str = "Unbound"

        try:
            response = perform_request(url)
            response_json: dict = json.loads(response.content)

            data: dict = read_json(output)

            if url not in data:
                data[url] = { get_utc_timestamp(): response_json }
            else:
                data[url][get_utc_timestamp()] = response_json

            write_json(output, data, indent=2)
        except Exception as e:
            message: str = f"Caught a {type(e)} exception while collecting samples: {e}, for URL {url}, outputting to file {output}, got response {response}"

            print(message)

            with open('./collector_errors.log', 'a+') as io:
                io.writelines([message])
        finally:
            print(f"Sleeping for {sleepy_time} seconds...")
            time.sleep(sleepy_time)


if __name__ == "__main__":

    token_file: str = "./auth.secret"

    if not os.path.isfile(token_file):
        print(f"Cannot find token file: {token_file}")
        raise SystemExit

    try:
        with open(token_file, "r") as f:
            token: str = f.read()
    except IOError as e:
        print(f"Cannot read token file: {token_file}")
        print(
            f"Forgot sudo? If that's not it, then `sudo chown root {token_file}` it and `sudo chmod ug-rwx {token_file}` immediately."
        )
        raise SystemExit

    length: int = len(token)
    token = token.strip()

    if len(token) != length:
        print(
            "WARNING: Whitespace got stripped from the end of the token file. You might want to get rid of that, or some requests may fail.\n"
        )

    operations: list = []

    for carg, narg in zip(sys.argv[1:], sys.argv[1 if len(sys.argv) >= 2 else 0 :]):
        if carg in ["--collect", "-c"]:
            operations.append(lambda url: debug_bg_collector(url, narg))
            print("Collector will run after prompting.")

        if carg in ['--help', '-h']:
            print("Just run it, or use --collect (-c) to gather debug samples from an API endpoint.")

    base: str = "https://api.github.com/repos"

    endpoints: list = [
        "traffic/views?per=week",
        "traffic/views?per=day",
        "traffic/clones?per=week",
        "traffic/clones?per=day",
        "traffic/popular/paths",
        "traffic/popular/referrers",
    ]

    print("Select an endpoint..\n")

    endpoint: str | None = None
    query: str = "\n?.): "

    while endpoint not in endpoints or endpoint is None:
        try:
            print("-" * max([len(s) for s in endpoints]))

            for index, endpoint in enumerate(endpoints):
                print(f"{index}.) {endpoint}")

            answer: str = input(query)

            if not len(answer):
                endpoint = None
                print("Typed nothing")
                continue

            endpoint_index = int(answer.strip())
            print("Index", endpoint_index)

            if endpoint_index < len(endpoints) and endpoint_index >= 0:
                endpoint = endpoints[endpoint_index]
                print("Endpoint set to", endpoint)
                break

        except ValueError as e:
            query = f"\n.. 0-9!!): "

        except IndexError as e:
            query = f"\n.. {len(endpoints)} > ?.): "

        except Exception as e:
            print(f"¯\\_(ツ)_/¯ : {e}")
            raise SystemExit

    owner: str = input("\n\nRepository Owner (default PsychedelicShayna): ")
    name: str = input("Repository Name (default github-chronicler): ")

    if not len(owner):
        owner = "PsychedelicShayna"

    if not len(name):
        name = "github-chronicler"

    request_url: str = f"{base}/{owner}/{name}/{endpoint}"
    print(f"\n\nUsing {request_url}..\n")

    for operation in operations:
        operation(request_url)

    response = perform_request(request_url)

    print(f"STATUS: {response.status_code}\n")
    print("---")
    print(f"{response.headers}")
    print("---")

    try:
        deserialized: dict = json.loads(response.content)
        print(json.dumps(deserialized, indent=4), end="\n\n")
    except json.JSONDecodeError as e:
        print(f"{response.content}\n")
        print("\n\nResponse content was not valid JSON\n---\n{e}")
    else:
        print("\n\nResponse content was formatted as JSON")

    print("\nNothing left to do.\n...Exiting...")

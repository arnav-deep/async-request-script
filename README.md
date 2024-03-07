# async-request-script
A Rust script to send requests in an async manner concurrently.

## Uses
- Testing async and multi-threading functionalites.
- Load testing endpoints

## Parameters
- `urls`: Vector of URLs to hit. Can be the same URL. The size of this vector is the number of requests that this script will send.
- `CONCURRENT_REQUESTS`: No of requests to send concurrently to the endpoint.
- `json_body`: The JSON body for the post request.

### Future Development
- Elapsed time for each individual request.
- Total Elapsed time.
- Options to take input from cli params.
- Make a library from this.
- Add CLI param options for POST/GET/PATCH/UPDATE requests, adding JSON or body, URLs, adding the number of requests to be sent to the URLs, concurrent requests.

- getBoundingClientRect
- addEventListener

## `Globalfetch.fetch` 

### Syntax

    fetch(input, init).then(function(response) { ... });
    
### Parameters
- input

    This defines the resource that you wish to fetch. This can either be:

    - A  [USVString](http://devdocs.io/dom/usvstring)  containing the direct URL of the resource you want to fetch.
    - A  [Request](http://devdocs.io/dom/request)  object.

- init Optional

    An options object containing any custom settings that you want to apply to the request. The possible options are:

    -  `method` : The request method, e.g., GET, POST.
    -  `headers` : Any headers you want to add to your request, contained within a Headers object or ByteString.
    -  `body` : Any body that you want to add to your request: this can be a Blob, BufferSource, FormData, URLSearchParams, or USVString object. Note that a request using the GET or HEAD method cannot have a body.
    -  `mode` : The mode you want to use for the request, e.g., cors, no-cors, or same-origin.
    -  `credentials` : The request credentials you want to use for the request: omit, same-origin, or include.
    -  `cache` : The cache mode you want to use for the request: default, no-store, reload, no-cache, force-cache, or only-if-cached.


### Returns

A Promise that resolves to a Response object.
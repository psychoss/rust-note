'use strict';

class Ajax {

	static req(url, options) {
		//  Setting the addition arguments.

		options = options || {};
		var method = options.method || 'GET';
		var headers;
		var body = options.body || '';
		if (options.headers) {
			headers = options.headers;
		} else {
			headers = new Headers();
			headers.append("content-type", "application/json");

		}



		var req = new Request(url);
		fetch(req, {
			method: method,
			headers: headers,
			body: body
		})
	}
}
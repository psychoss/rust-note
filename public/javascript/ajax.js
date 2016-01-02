'use strict';

class Ajax{
	
	static req(url,options){
		//  Setting the addition arguments.
		
		options=options||{};
		var method=options.method||'GET';
		
		
		
		var req=new Request(url);
		fetch(req,{
		method:method
		})
	}
}
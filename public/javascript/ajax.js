'use strict';

class Ajax{
	
	static get_from(url,options){
		//  Setting the addition arguments.
		
		options=options||{};
		var method=options.method||'GET';
		
		
		
		var req=new Request(url);
		fetch(req,{
		
		})
	}
}
'use strict';
    /**
	 * ------------------------------------------------------------------------
	 *  Global Variables:
	 * Ajax 
	 * ------------------------------------------------------------------------
	 */
class Exchange{
	
	constructor(){
		save.addEventListener('click',function(){
			Ajax.req("/push",{
			method:'POST'
		})
		})
	}
	save(data){
		Ajax.req("/push",{
			method:'POST'
		})
	}
	
}
new Exchange;
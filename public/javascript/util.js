'use strict';

class Util{
	/**
	 * 
	 *  For Title
	 */
	static getTitle(){
	return	document.title;
	}
	static setTitle(title){
		if(title){
			document.title=title;
		}
	}
	/**
	 * 
	 * For Manage ClassName
	 */
	static setClass(ele,name){
		if(ele&&!ele.classList.contains(name)){
			ele.classList.add(name);
		}
	}
	static hasClass(ele,name){
		return ele.classList.contains(name);
	}
	
	
}
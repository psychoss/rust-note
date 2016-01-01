'use strict';

class TextProcessor {

	static setHeader(str) {
		if (typeof str==='undefined')return;
		// “ ” - 32
		//“#” - 35
		let first_char = str.codePointAt(0);
		if (first_char == 35 || first_char == 32) {
			return '#'.concat(str);
		} else {
			return '# '.concat(str);
		}
	}

}
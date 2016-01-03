'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 *  TextProcessor 
 * ------------------------------------------------------------------------
 */
class TextProcessor {

	static bold(str) {
		str = str.trim();
		if (/^\*{2}[^\n]*\*{2}$/.test(str)) {
			return [].slice.call(str, 2, str.length - 2).join("");
		} else {
			return " **" + str + "** ";
		}
	}
	static italic(str) {
		str = str.trim();
		if (/^\*[^\n]*\*$/.test(str)) {
			return [].slice.call(str, 1, str.length - 1).join("");
		} else {
			return " *" + str + "* ";
		}
	}
	static link(str) {
		return [" [", str.trim(), "]() "].join("");
	}
	static code(str) {
		if (/\n/.test(str)) {
			return ["\n", "```", "\n", str, "\n", "```", "\n"].join("");
		}else{
			return [ " `", str.trim(), "` "].join("");			
		}
	}

}
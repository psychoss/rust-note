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
		if (!str.trim() || /\n/.test(str)) {
			return ["\n", "```", "\n", str, "\n", "```", "\n"].join("");
		} else {
			return [" `", str.trim(), "` "].join("");
		}
	}
	static li(str) {
		var list = str.trim().split('\n');
		var arr = [];
		for (var index = 0; index < list.length; index++) {
			arr.push("- " + list[index].trim());
		}
		return arr.join("\n");
	}
	static ol(str) {
		var list = str.trim().split('\n');
		var arr = ['\n'];
		for (var index = 0; index < list.length; index++) {
			arr.push((index + 1) + ". " + list[index].trim() + "\n");
		}
		arr.push('\n');
		return arr.join('');
	}
	static getFirstLine(str) {
		str = str.trim();
		if (str) {
			return str.split('\n')[0];
		}
	}
	static formatToArray(str) {
		var sl = str.trim().replace(/\"/g, "\\\"").split('\n');
		var arr = [];
		for (var index = 0; index < sl.length; index++) {
			arr.push('"' + sl[index] + '“');
		}
		return '[' + arr.join() + "]";
	}
	static sort(str) {
			var sl = str.trim().split('\n');
			sl.sort(function(a, b) {
				return a.trim() > b.trim();
			});
			return sl.join('\n');
		}
		/**
		 * 
		 * Get CodePoint of first character.
		 * 
		 */
	static getCodePoint(str) {
		var l = str.length;
		var result = "";
		for (var index = 0; index < l; index++) {
			result += '&#' + str.codePointAt(index) + ";";
		}
		return result;
	}
	static sortObject(str) {
		return str.replace(/`/g,"&#96;");
		// var array = str.split('');
		// var collect = [];
		// var line = "";
		// var count = 0;
		// for (var c of array) {
		// 	if (c === "{") {
		// 		count += 1;
		// 		line += c;

		// 	} else if (c === "}") {
		// 		count -= 1;
		// 		line += c;

		// 	} else if (c === "," && count === 0) {
		// 		collect.push(line);
		// 		line = "";
		// 	} else {
		// 		line += c;
		// 	}


		// }
		// var re = /name:\s+\"([^\"]*)\"/;

		// console.log('complains：', collect);
		// collect.sort(function(a, b) {
		// 	var _a = re.exec(a)[1].trim();
		// 	var _b = re.exec(b)[1].trim();

		// 	if (_a > _b)
		// 		return 1;
		// 	if (_a < _b)
		// 		return -1;
		// 	return 0;
		// });

		// return collect.join();

	}
}
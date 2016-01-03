'use strict';

class Util {
	/**
	 * 
	 *  For Title
	 */
	static getTitle() {
		return document.title;
	}
	static setTitle(title) {
			if (title) {
				document.title = title;
			}
		}
		/**
		 * 
		 * For Manage ClassName
		 */
	static addClass(ele, name) {
		if (ele && !ele.classList.contains(name)) {
			ele.classList.add(name);
		}
	}
	static removeClass(ele, name) {
		if (ele && ele.classList.contains(name)) {
			ele.classList.remove(name);
		}
	}
	static hasClass(ele, name) {
		return ele.classList.contains(name);
	}

	static html(ele, value) {
		if (!ele) return;
		if (value) {
			ele.innerHTML = value;
		} else {
			return ele.innerHTML;
		}
	}

	static template(template, json) {
		var delimiter = /{([a-zA-Z_\-0-9]+)}/g;

		var line = function(data) {
			return template.replace(delimiter, function(m, g) {
				if (g.startsWith("-")) {
					g = g.slice(1);
					return data[g] ? '' : g;
				}
				return data[g] || '';
			})
		};


		var content = "";

		for (var index = 0, l = json.length; index < l; index++) {
			content += line(json[index]);
		}

		return content;
	}

}
'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 * Util 
 * ------------------------------------------------------------------------
 */
class Dropdown {

	constructor(selector) {
		this.element = document.querySelector(selector);
		this._configure();


	}

	_configure() {
		if (!this.element) {
			console.log("Dropdown class complains：", 'missing the element');
			return;
		}
		Util.hiddenByClick(this.element, "is-visible");
		let target = this.element.getAttribute('data-target');
		if (!target) {
			console.log('Drop down class complains：', 'missing target');
			return;
		}
		let targetElement = document.querySelector(target);
		if (!targetElement) {
			return;
		}
		this._calculate(targetElement);
		this._bindEvent(targetElement);

	}
	_bindEvent(target) {
		let self = this;
		target.addEventListener('click', function(ev) {
			ev.stopImmediatePropagation();
			Util.addClass(self.element, "is-visible");
		});
	}
	_calculate(target) {
		let rects = target.getBoundingClientRect();
		let styles = [];

		// set top
		styles.push("top:");
		styles.push(rects.top + rects.height + 3);
		styles.push("px;");

		// set left
		styles.push("left:");
		styles.push(rects.left);
		styles.push("px");


		this.element.style.cssText = styles.join("");
		console.log('complains：', rects);
	}

}
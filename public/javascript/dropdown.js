'use strict';

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

	}
	_bindEvent(){
		
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
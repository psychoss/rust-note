'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 * Util 
 * ------------------------------------------------------------------------
 */
class Notifier {

	constructor(params) {
		this._configure();
	}
	_configure() {
		let n = document.createElement('div');
		n.className = 'notification';
		document.body.appendChild(n);
		this.notifier = n;
	}

	notify(message) {
		Util.html(this.notifier, message);
		Util.addClass(this.notifier, 'is-visible');
		clearTimeout(this.timeout);
		let self=this;
		this.timeout = setTimeout(function() {
			Util.removeClass(self.notifier, 'is-visible')
		}, 2000);
	}
}
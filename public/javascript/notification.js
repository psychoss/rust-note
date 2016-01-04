'use strict';

class Notifier {
	static init() {
		let n = document.createElement('div');
		n.className = 'notification';
		document.body.appendChild(n);
		Notifier.n = n;
	}
	static notify(message) {
		Util.html(Notifier.n,message);
		Util.addClass(Notifier.n, 'is-visible');
	   	setTimeout(function() {
			Util.removeClass(Notifier.n,'is-visible')
		}, 2000);
	}
}
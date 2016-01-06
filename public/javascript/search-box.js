'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 *  catSelect(id) 
 * ------------------------------------------------------------------------
 */
class SearchBox {
	constructor(editor, notifier) {
		this.editor = editor;
		this.notifier = notifier;
		this.searchBox = document.querySelector('.search-box');
		this.showTrigger = document.querySelector('.js-notelist');
		this.searchInput = document.querySelector('.search-input-container input');


		this.searchInput.addEventListener('click', function(ev) {
			ev.stopImmediatePropagation();

		})

		this.showTrigger.addEventListener('click', function(ev) {
			ev.stopImmediatePropagation();
			Util.addClass(this.searchBox, 'is-visible');
		}.bind(this))

		document.addEventListener('click', function() {
			Util.removeClass(this.searchBox, 'is-visible');
		}.bind(this));
		this._bindSelect();
	}
	_bindSelect() {

		let self = this;
		catSelect.addEventListener('click', function(ev) {
			ev.stopImmediatePropagation();

		});
		catSelect.addEventListener('change', function(ev) {
			if (catSelect.value === "Notes") {
				self.refresh();
			} else {
				self.refresh_by(catSelect.value);
			}
		})
	}
	_bindClick() {
		var self = this;

		let elements = document.querySelectorAll(".search-list-container a");
		var l = elements.length;
		for (; l--;) {
			elements[l].addEventListener("click", function(ev) {
				var target = ev.target;
				var id = target.getAttribute('data-id');
				document.body.setAttribute("data-id", id);
				document.title = target.textContent;
				self._getContent(id);
			});
		}
	}
	_getContent(id) {
		var self = this;

		Ajax.req("/query-one", {
			method: "POST",
			body: JSON.stringify({
				_id: id
			})
		}).then(function(res) {
			res.text().then(function(v) {
				try {

					self.editor.setText(v);

				} catch (error) {

				}
			})
		}).catch(function() {
			self.notifier.notify("Failed");
		});
	}
	refresh_by(cat) {
		var self = this;

		Ajax.req("/query-cat", {
			method: "POST",
			body: JSON.stringify({
				cat: cat
			})
		}).then(function(res) {
			res.text().then(function(v) {
				try {
					let content = Util.template("<a data-id=\"{id}\">{title}</a>", JSON.parse(v));
					Util.html(document.querySelector('.search-list-container'), content);
					self._bindClick();
				} catch (error) {
					self.notifier.notify(error);
				}
			})
		}).catch(function() {

		});
	}
	refresh() {
		var self = this;

		Ajax.req("/query", {
			method: "POST"
		}).then(function(res) {
			res.text().then(function(v) {
				try {
					let content = Util.template("<a data-id=\"{id}\">{title}</a>", JSON.parse(v));
					Util.html(document.querySelector('.search-list-container'), content);
					self._bindClick();
				} catch (error) {
					self.notifier.notify(error);

				}
			})
		}).catch(function() {
			self.notifier.notify("Failed");
		});
	}
}
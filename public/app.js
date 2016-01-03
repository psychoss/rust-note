'use strict';

const editor = new Editor;
const cmd_bridge = new CommandBridge(editor);
cmd_bridge.bindElement();



function configureMarkdown() {
	marked.setOptions({
		highlight: function(code) {
			return code.replace(/[<>]/g, function(m) {
				if (m === '<')
					return '&lt;'
				else if (m === '>')
					return '&gt;'
			}).replace(/(^|[^\\])\/\*[\w\W]*?\*\//g, function(m) {
				return '<span class="comment">' + m + '</span>';
			}).replace(/(^|[^\\:])\/\/.*/g, function(m) {
				return '<span class="comment">' + m + '</span>';
			})
		}
	});
}
configureMarkdown();

function autoSize() {
	preview.style.height = (window.innerHeight - 33) + 'px';

	window.addEventListener('resize', function() {
		preview.style.height = (window.innerHeight - 33) + 'px';
	})
}
autoSize();

new Dropdown('.js-dropdown-menu');
new Dropdown('.js-dropdown-notelist');

new Exchange(editor);

class SearchBox {
	constructor(editor) {
		this.editor = editor;
		this.searchBox = document.querySelector('.search-box');
		this.showTrigger = document.querySelector('.js-notelist');

		this.showTrigger.addEventListener('click', function(ev) {
			ev.stopImmediatePropagation();
			Util.addClass(this.searchBox, 'is-visible');
		}.bind(this))

		document.addEventListener('click', function() {
			Util.removeClass(this.searchBox, 'is-visible');
		}.bind(this))
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
				console.log('complains：',v);
				try {
					
					self.editor.setText(v);

				} catch (error) {

				}
				console.log('complains：', v);
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

				}
				console.log('complains：', v);
			})
		}).catch(function() {

		});
	}
}

const searchBox = new SearchBox(editor);

searchBox.refresh();
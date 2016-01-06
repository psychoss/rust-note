'use strict';

const editor = new Editor;
const notifier = new Notifier;
const searchBox = new SearchBox(editor,notifier);
const exchange = new Exchange(editor, searchBox,notifier);
const cmd_bridge = new CommandBridge(editor, exchange);
new Dropdown('.js-dropdown-menu');

cmd_bridge.bindElement();
searchBox.refresh();



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

document.addEventListener('keydown', function(ev) {
	var k = (ev.which || ev.keyCode);
	// Prevent page backforward on pressing BackSpace.
	if (k === 8 && ev.target.tagName !== 'INPUT') {
		ev.preventDefault();
	}
	// Prevent page refresh on pressing F5.
	if (k === 116) {
		console.log(ev)
		ev.preventDefault();
	}
});
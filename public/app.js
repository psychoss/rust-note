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
new Exchange(editor);
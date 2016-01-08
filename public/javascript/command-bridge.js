'use strict';

/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 *  TextProcessor 
 * ------------------------------------------------------------------------
 */

class CommandBridge {
	constructor(editor, exchange) {
		this.editor = editor;
		this.exchange = exchange;
		this._configure();


	}
	_configure() {
		if (this.editor) {
			let editor = this.editor.aceditor;
			let cmd = this._commandBuild();
			let l = cmd.length;
			for (; l--;) {
				editor.commands.addCommand(cmd[l])
			}
		}
	}

	bindElement() {
		let elements = document.querySelectorAll('[command]');
		let l = elements.length;
		var self = this;
		for (; l--;) {
			elements[l].addEventListener('click', function(ev) {
				self.editor.aceditor.execCommand(ev.currentTarget.getAttribute('data-exec'));
			})
		}
	}
	_commandBuild() {
		var self = this;
		return [{
			name: "bold",
			bindKey: {
				win: "F3"
			},
			exec: self._toggleBoldMarker
		}, {
			name: "code",
			bindKey: {
				win: "F2"
			},
			exec: self._addCodeMarker
		}, {
			name: "codePoint",
			exec: self._getCodePoint
		}, {
			name: "header",
			bindKey: {
				win: "F1"
			},
			exec: self._addTitleMarker
		}, {
			name: "hidden",
			exec: self._hidden
		}, {
			name: "hr",
			exec: self._addHrMarker
		}, {
			name: "italic",
			exec: self._toggleItalicMarker
		}, {
			name: "li",
			exec: self._addListMarker
		}, {
			name: "link",
			bindKey: {
				win: "F4"
			},
			exec: self._addLinkMarker
		}, {
			name: "new",
			exec: self._newCommand.bind(self)
		}, {
			name: "ol",
			exec: self._addOlMarker
		}, {
			name: "save",
			bindKey: {
				win: "F5"
			},
			exec: self._saveCommand.bind(self)
		}, {
			name: "sort",
			exec: self._sort
		}, {
			name: "sortObject",
			exec: self._sortObject
		}, {
			name: "toarray",
			exec: self._toarray
		}]

	}
	_addTitleMarker(e) {
		let range = e.getSelectionRange().collapseRows();
		let doc = e.session.doc;
		let line = doc.getLine(range.start.row)
		if (/^#* /.test(line)) {
			doc.insertInLine({
				row: range.start.row,
				column: 0
			}, "#");
		} else {
			doc.insertInLine({
				row: range.start.row,
				column: 0
			}, "# ");
		}
	}
	_toggleBoldMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.bold(str);
		console.log(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_toggleItalicMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.italic(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_addLinkMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.link(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_addCodeMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.code(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_addListMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.li(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_addOlMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.ol(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_addHrMarker(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = str + "\n\n-----\n\n";
		e.session.replace(e.getSelectionRange(), str);
	}
	_saveCommand() {
		this.exchange.save();
	}
	_newCommand() {
		document.body.removeAttribute("data-id");
		document.title = "New Note";
		this.editor.setText("");
	}
	_toarray(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.formatToArray(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_sort(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.sort(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_hidden() {
		let editor = document.querySelector('.editor');
		let markdownContainer = document.querySelector('.markdown-container');
		if (!Util.hasClass(editor, 'hidden')) {
			Util.addClass(editor, 'hidden');
			Util.addClass(markdownContainer, 'full-width');
		} else {
			Util.removeClass(editor, 'hidden');
			Util.removeClass(markdownContainer, 'full-width');
		}
	}
	_getCodePoint(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.getCodePoint(str);
		e.session.replace(e.getSelectionRange(), str);
	}
	_sortObject(e) {
		let str = e.session.getTextRange(e.getSelectionRange())
		str = TextProcessor.sortObject(str);
		e.session.replace(e.getSelectionRange(), str);
	}
}

// selectedText() {
// 		return this.editor.session.getTextRange(this.editor.getSelectionRange())
// 	}

// 	replaceSelectedText(str) {
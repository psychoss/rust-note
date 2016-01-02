'use strict';

/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 *  TextProcessor 
 * ------------------------------------------------------------------------
 */

class CommandBridge {
	constructor(editor, textProcessor) {
		this.editor = editor;
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
				name: "help",
				bindKey: {
					win: "F2"
				},
				exec: function() {

				}
			}, {
				name: "header",
				bindKey: {
					win: "F1"
				},
				exec: self._addTitleMarker
			}, {
				name: "bold",
				bindKey: {
					win: "F3"
				},
				exec: self._toggleBoldMarker
			}, {
				name: "italic",
				exec: self._toggleItalicMarker
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

	}

	// selectedText() {
	// 		return this.editor.session.getTextRange(this.editor.getSelectionRange())
	// 	}

	// 	replaceSelectedText(str) {
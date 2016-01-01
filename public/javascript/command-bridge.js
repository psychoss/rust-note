'use strict';

class CommandBridge {
	constructor(editor, textProcessor) {
		this.editor = editor;
		this._configure();
		this.processor = textProcessor;

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
	_commandBuild() {
		var self = this;
		return [{
			name: "help",
			bindKey: {
				win: "F1"
			},
			exec: function() {

			}
		}, {
			name: "header",
			bindKey: {
				win: "Ctrl+H"
			},
			exec: self._addTitleMarker
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

}
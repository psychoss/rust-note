'use strict';

class Editor {
	constructor() {
		this._configure();
		this._initEditor();
	}
	get aceditor() {
		return this.editor;
	}
	_configure() {
		this.container = document.querySelector('.editor');
		if (!this.container) {
			throw new Error('missing editor element');
		}
	}
	_initEditor() {
		this.editor = ace.edit(this.container);
		this.editor.$blockScrolling = Infinity;
		this.editor.setShowPrintMargin(false);
		this.editor.getSession().setMode('markdown');
	}
}
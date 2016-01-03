'use strict';
    /**
	 * ------------------------------------------------------------------------
	 *  Global Variables:
	 *  
	 * marked
	 * save (id)
	 * ------------------------------------------------------------------------
	 */
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
		this.editor.getSession().setMode('ace/mode/markdown');
		this.editor.setOption("wrap", true);
		this._bindContentChanged();
		
	}
	_bindContentChanged(){
		this.markdownBody=document.querySelector('.markdown-body');
		var self=this;
		if (this.markdownBody){
			this.editor.on('change',function(_,e){
				 Util.setClass(save,"danger");
				self.markdownBody.innerHTML=marked(e.getValue().trim());
			});
		}		
	}
	getText(){
		return this.editor.getValue();
	}
	setText(str){
		this.editor.setValue(str);
	}
	selectedText() {
		return this.editor.session.getTextRange(this.editor.getSelectionRange())
	}

	replaceSelectedText(str) {
		this.editor.session.replace(this.editor.getSelectionRange(), str);
	}
	
}
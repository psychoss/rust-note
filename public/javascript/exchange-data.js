'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 * Ajax 
 * TextProcessor 
 * ------------------------------------------------------------------------
 */
class Exchange {

	constructor(editor) {
		this.editor = editor;

		save.addEventListener('click', this.save.bind(this));
	}
	_getBody() {

	}
	save() {
		var options = {
			method: 'POST'
		}
		var data = {};
		data.content = this.editor.getText();
		data.title = TextProcessor.getFirstLine(data.content);
		if (!data.title) {
			return;
		} else {
			data.title = data.title.replace(/^#* */, '').trim();
		}
		data.create = Date.now();
		data.modified = Date.now();
		try {
			options.body = JSON.stringify(data)

		} catch (e) {

		}

		console.log('complains：', options);

		Ajax.req("/push",options);
	}

}
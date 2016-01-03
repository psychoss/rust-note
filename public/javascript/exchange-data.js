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
	_getId() {
		return document.body.getAttribute('data-id') || 0;
	}
	_setId(id) {
		document.body.setAttribute('data-id', id);
	}
	save() {
		var options = {
			method: 'POST'
		}
		var data = {};
		data._id = this._getId();
		data.content = this.editor.getText();
		data.title = TextProcessor.getFirstLine(data.content);
		if (!data.title) {
			return;
		} else {
			data.title = data.title.replace(/^#* */, '').trim();
		}
		if (data._id === 0) {
			data.create = Date.now();
		}
		data.modified = Date.now();
		try {
			options.body = JSON.stringify(data)
		} catch (e) {

		}

		var self = this;

		if (data._id === 0) {
			Ajax.req("/push", options).then(function(res) {
				res.text().then(function(v) {
					self._setId(v);
Util.removeClass(save,"danger");
					
				})
			}).catch(function() {

			});
		} else {
			console.log('update the database use => ',options);
			Ajax.req("/update", options).then(function(res) {
				res.text().then(function(v) {
Util.removeClass(save,"danger");
				})
			}).catch(function() {

			});
		}
	}

}
'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 * Ajax 
 * TextProcessor 
 * Notifier
 * ------------------------------------------------------------------------
 */
class Exchange {

	constructor(editor, searchBox) {
		this.editor = editor;
		this.searchBox = searchBox;


	}
	_getId() {
		return document.body.getAttribute('data-id') || 0;
	}

	_update(title, id) {

		if (id)
			document.body.setAttribute('data-id', id);


		Util.removeClass(save, "danger");
		if (document.title !== title) {
			//this.searchBox.refresh();
			document.title = title;
		}
		Notifier.notify("Success");

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
		data.cat = catSelect.value;
		if (data.cat === "Notes") {
			data.cat = "";
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
					self._update(data.title, v);
				})
			}).catch(function() {
				Notifier.notify("Failed.");
			});
		} else {
			console.log('update the database use => ', options);
			Ajax.req("/update", options).then(function(res) {
				res.text().then(function(v) {
					self._update(data.title);
				})
			}).catch(function() {
				Notifier.notify("Failed.");
			});
		}
	}

}
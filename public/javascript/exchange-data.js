'use strict';
/**
 * ------------------------------------------------------------------------
 *  Global Variables:
 * Ajax 
 * TextProcessor 
 * 
 * ------------------------------------------------------------------------
 */
class Exchange {

	constructor(editor, searchBox, notifier) {
		this.editor = editor;
		this.searchBox = searchBox;
		this.notifier = notifier;


	}
	_getId() {
		return document.body.getAttribute('data-id') || 0;
	}

	_update(title, id, cat) {

		if (id)
			document.body.setAttribute('data-id', id);


		Util.removeClass(save, "danger");
		if (document.title !== title) {
			if (!cat)
				this.searchBox.refresh();
			else {
				this.searchBox.refresh_by(cat);
				catSelect.querySelector('input').value = cat;
			}
			document.title = title;
		}
		this.notifier.notify("Success");

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
		data.cat = catSelect.querySelector('input').value;
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
					self._update(data.title, v, data.cat);
				})
			}).catch(function() {
				self.notifier.notify("Failed.");
			});
		} else {
			console.log('update the database use => ', options);
			Ajax.req("/update", options).then(function(res) {
				res.text().then(function(v) {
					self._update(data.title, data.cat);
				})
			}).catch(function() {
				self.notify("Failed.");
			});
		}
	}

}
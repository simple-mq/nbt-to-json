import React, { Component } from 'react';
import './JsonView.css';
//@ts-ignore
import { saveAs } from 'file-saver';
//@ts-ignore
import { get_json } from 'crate';
import ReactJson from 'react-json-view';

interface IJsonView {
	src: object;
	_src: string;
	file_name: string;
}

class JsonView extends Component<IJsonView, IJsonView> {
	constructor(props: IJsonView) {
		super(props);
		this.state = props;
	}

	displayJson(json: string, file_name: string) {
		this.setState({
			src: JSON.parse(json),
			_src: json,
			file_name
		});
	}

	render(): React.ReactNode {
		return (
			<div className='JsonView' id='JsonView' style={{ width: isMobile() ? '90%' : '70%', fontSize: isMobile() ? '12px' : '15px' }}>
				<Button
					upload_event={(e: React.ChangeEvent<HTMLInputElement>) =>
						get_json(e.target.files![0]).then((json) => this.displayJson(json, e.target.files![0].name))
					}
					download_event={() => saveAs(new Blob([this.state._src], { type: 'text/plain;charset=utf-8' }), this.state.file_name + '.json')}></Button>
				<div id='output'>
					<ReactJson
						src={this.state.src}
						name={null}
						theme='harmonic'
						displayDataTypes={false}
						indentWidth={isMobile() ? 1 : 8}
						style={{ fontFamily: 'sans-serif' }}
					/>
				</div>
			</div>
		);
	}
}

interface IButton {
	upload_event: (e: React.ChangeEvent<HTMLInputElement>) => void;
	download_event: (e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void;
}

function Button(props: IButton) {
	return (
		<div id='button-div'>
			<div className='op-div'>
				<button id='upload' className='op-button'>
					<label className='text'>上传文件</label>
				</button>
				<input type='file' name='input-file' id='input' onChange={props.upload_event} />
			</div>
			<div className='op-div'>
				<button id='download' className='op-button' onClick={props.download_event}>
					<label className='text' htmlFor='download'>
						下载文件
					</label>
				</button>
			</div>
		</div>
	);
}

function isMobile() {
	const flag = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
	return flag;
}

export default JsonView;

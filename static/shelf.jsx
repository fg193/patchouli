function App() {
	return shelves.map(props => <Shelf {...props} />);
}

function Shelf(props) {
	return (
		<div class="shelf col">
			<h4 class="shelf-name">{props.name}</h4>
			<table class="shelf-main"><tbody>
				{props.cells.trim().split('\n').map(line => <ShelfRow line={line} />)}
			</tbody></table>
		</div>
	);
}

function ShelfRow(props) {
	return (
		<tr class="shelf-row">
			{props.line.trim().split(/\s+/).map(cell => cell && <ShelfCell cell={cell} />)}
		</tr>
	);
}

function ShelfCell(props) {
	const [name, classes] = props.cell.split('.');
	return (
		<td class={classes && classes.split('').join(' ')}>
			{props.scope}{name}
		</td>
	);
}

const container = document.getElementById('root');
ReactDOM.createRoot(container).render(<App />);

import TopBar from "../components/topbar/topbar";
import "./styles/home.css"

export default function Home() {
	return <>
		<TopBar />

		<div className="main">
			<div className="title-header">
				<img src="src/assets/IgniteIcon.svg" alt="Ignite Image" />

				<h1>
					<span className="japanese">点火す</span>
					Ignite
				</h1>
			</div>

			<div className="description">
				A dynamically typed bytecode compiled language made
				as a hobby project. <br /> Inspired by Rust, Python, C#, and JS.
			</div>

			<div className="button-holder">
				<a className="primary" href="docs"> Documentation </a>
				<a href="api"> API </a>
			</div>
		</div>
	</>;
}

import "./topbar.css"

export default function TopBar() {
	return <div className="topbar">
		<a className="title" href="/">
			<img src="src/assets/IgniteIcon.svg" alt="IgniteIcon" />
			<div className="text">Ignite</div>
		</a>

		<div className="navigation">
			<a href="docs">Documentation</a>
			<a href="api">API</a>
			<a href="https://github.com/creepersaur/ignite">
				<img src="src/assets/GithubIcon.svg" />
			</a>
		</div>
	</div>
}
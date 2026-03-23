import { Link } from 'react-router-dom'
import "./topbar.css"
import icon from '../../assets/IgniteIcon.svg'
import GithubIcon from '../../assets/GithubIcon.svg'

export default function TopBar() {
	return <div className="topbar">
		<Link className="title" to="/">
			<img src={icon} alt="IgniteIcon" />
			<div className="text">Ignite</div>
		</Link>

		<div className="navigation">
			<Link to="/docs">Documentation</Link>
			<Link to="/api">API</Link>
			<a href="https://github.com/creepersaur/ignite">
				<img src={GithubIcon} />
			</a>
		</div>
	</div>
}
import { render } from 'preact';
import { LocationProvider, Router, Route } from 'preact-iso';

import './style.css';
import { Home } from './pages/Home';

export function App() {
	  return (
		<div className="app">
		  <main>
			<Home/>
		  </main>
		</div>
	  );
	};
	
render(<App />, document.getElementById('app'));

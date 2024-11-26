import { useEffect, useState } from 'preact/hooks';
import preactLogo from '../../assets/preact.svg';
import './style.css';
import { ThreadSelector } from '../../components/ThreadSelctor';
import { Post } from '../../components/Post';
import { ThreadList } from '../../components/ThreadList';

export function Home() {

	const [threads, setThreads] = useState([]);
	const [posts, setPosts] = useState([]);
	const [selectedThread, setSelectedThread] = useState<string>(null);

  
	console.log("thread:", selectedThread);

	return (
		<div class="home">
			<div className="sidebar">
			  <h2>Threads</h2>
			  <ThreadList onSelectThread={setSelectedThread}/>
			</div>
			<div className="content">
			</div>
		</div>
	);
}


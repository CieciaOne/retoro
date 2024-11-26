import axios from 'axios';
import { useEffect, useState } from 'preact/hooks';
import { ThreadSelector } from './ThreadSelctor';

export const Thread =({onSelectThread})=> {
    const [data, setData] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);
  
    // const onSelectThread = (id: String) => { console.log(id);

    // };
    useEffect(() => {
      // Define an async function to fetch data
      const fetchData = async () => {
        try {
          setLoading(true); // Start loading
          const response = await axios.get('http://localhost:8080/api/post')
          setData(response.data); // Save the data
        } catch (err) {
          setError(err.message); // Save the error
        } finally {
          setLoading(false); // End loading
        }
      };
  
      fetchData();
    }, []); // Empty dependency array means this runs once on mount
  
    // Conditional rendering based on state
    if (loading) return <p>Loading...</p>;
    if (error) return <p>Error: {error}</p>;
    console.log(data);
    return(
        <div>
            {data.map(thread => {
                return <ThreadSelector thread={thread} onSelectThread={onSelectThread}/>
            })}
        </div>
    )
  


}

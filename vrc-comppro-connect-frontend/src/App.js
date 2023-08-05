import React, { useEffect, useState } from 'react';
import './App.css';
import {axios} from 'axios'

function App() {
  const url = "http://localhost/";
  const [count, setCount] = useState(0)

  useEffect(() => {
    axios.get(url).then((res) => {
      console.log(res.data);
    }).catch(error => {
      console.log(error);
    });
  }, [])
  return (
    <div className="App">
      helloworldf2asdfa
    </div>
  )
}

export default App;

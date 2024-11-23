import React, { useEffect, useState } from 'react';

function App() {
  const [message, setMessage] = useState('');

  useEffect(() => {
    fetch('http://127.0.0.1:8080/api/hello')
        .then((response) => response.json())
        .then((data) => setMessage(data.content))
        .catch((error) => console.error('Error fetching message:', error));
  }, []);

  const handleEcho = async () => {
    const response = await fetch('http://127.0.0.1:8080/api/echo', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content: 'Hello from React!' }),
    });
    const data = await response.json();
    alert(data.content);
  };

  return (
      <div>
        <h1>Rust + React</h1>
        <p>{message}</p>
        <button onClick={handleEcho}>Send Echo</button>
      </div>
  );
}

export default App;

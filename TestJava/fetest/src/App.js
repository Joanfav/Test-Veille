import React, { useEffect, useState } from 'react';

function App() {
    const [message, setMessage] = useState('');

    useEffect(() => {
        fetch('http://localhost:8080/hello')
            .then((response) => {
                if (!response.ok) {
                    throw new Error('Network response was not ok');
                }
                return response.json();
            })
            .then((data) => setMessage(data.content))
            .catch((error) => console.error('Error fetching message:', error));
    }, []);

    const handleEcho = async () => {
        try {
            const response = await fetch('http://localhost:8080/echo', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ content: 'Hello from React!' }),
            });

            if (!response.ok) {
                throw new Error('Network response was not ok');
            }

            const data = await response.json();
            alert(data.content);
        } catch (error) {
            console.error('Error sending echo:', error);
        }
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

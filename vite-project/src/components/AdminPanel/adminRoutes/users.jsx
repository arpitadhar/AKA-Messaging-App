import React, {useState, useEffect} from 'react'; 
import axios from 'axios'; 


export default function Users(){
    const [users, setUsers] = useState([]); 
    useEffect(() => {
        const getUsers = async () => {
            try{
                const { data } = await axios.get("http://localhost:3000/users"); 
                setUsers(data); 
                console.log(data); 
            }
            catch(error){
               console.log(error); 
            }
        }
        getUsers(); 
    }, [])
    return(
        <div>Users
           {users}
           {users.username}
            { /*<ul>
                {users.map(user => (
                    <li key={user.id}>
                        <strong>Username:</strong> {user.username}, 
                        <strong> First Name:</strong> {user.first_name}, 
                        <strong> Last Name:</strong> {user.last_name}
                    </li>
                ))}
                </ul> */ }
        </div>
    )
}
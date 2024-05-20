import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import "./adminPanel.css"; 
import SidebarAdmin from "../SidebarAdmin/SidebarAdmin";
import adminpng from "./admin.png"; 
import { RiAdminFill } from "react-icons/ri";
import toast from 'react-hot-toast'; 
//import { MDBCol, MDBContainer, MDBRow, MDBCard, MDBCardTitle, MDBCardText, MDBCardBody, MDBCardImage, MDBBtn } from 'mdb-react-ui-kit';
import { Menu } from "antd"; 
import { Button, Flex, Spin } from "antd"; 
//import { useNavigate } from "react-router-dom";
export default function AdminPanel(props) {
    //console.log(props.user);
    const [loading, setLoading] = useState(false);
    const [users, setUsers] = useState([]); 
    const usernameDefault = localStorage.getItem("actual_email");
    const user = usernameDefault.replace(/^"(.*)"$/, '$1');
    useEffect(() => {
        const getUsers = async () => {
            setLoading(true); 
            try {
                const payload = { user};
                console.log("Sending payload:", payload);

                const res = await fetch("http://localhost:3000/users", {
                    method: 'GET', 
                    headers: {
                        'Content-Type': 'application/json'
                    }, 
                }); 

                if (!res.ok) {
                    const errorText = await res.text();
                    throw new Error(`Error ${res.status}: ${errorText}`);
                }

                const data = await res.json(); 
                if (data.error) {
                    throw new Error(data.error); 
                }
                setUsers(data); 
            } catch (error) {
                toast.error(error.message); 
            } finally {
                setLoading(false); 
            }
        }
        getUsers();
    }, [user]); 
    const transformUsers = (data) => {
        if (Array.isArray(data)) {
            return data; 
        } else if (typeof data === 'string') {
            return JSON.parse(data);
        } else if (data &&  typeof data === 'object') {
            return [data]; 
        } else {
            throw new Error("Invalid data format received.");
        }
    };
    console.log("conversations", transformUsers(users));
    return (
    // <div class="adminBody">
    //   <div className="pageAdmin">
    //     <SidebarAdmin/>
    //     <div className="nameAdmin">
    //         <h1>Admin Panel</h1>
    //     </div>
    //     <div>
    //     </div>
    //   </div>
    //   </div>
    <div class="grid-container" style={{ fontFamily: 'Rubik, sans-serif' }}>
      <div class="appHeader">
       <h1> <RiAdminFill /> Admin Panel </h1>
      </div>
    <div className="space">
    <div class = "SideMenu"> 
    <Menu className="menu1"
    onClick={(item) => {

    }}
    items={[
        {
        label: "Users", 
        key: "chat/adminpanel", 
     }, 
     {
        label: "Flagged", 
        key: "/Flagged", 
     }, 
      
        ]}></Menu>
    </div>
    <div class="pageContent"> 

    {transformUsers(users).map((registered_user) => ( //add ,idx if needed
              <div className="boxes"><div className="inside"><div className = "pfp"></div><div className = "user-name"> <div>{registered_user.first_name} {registered_user.last_name}</div><div>{registered_user.email}</div><div className = "user-buttons"> <Button danger>Delete User</Button><br></br><div></div><Button type="primary">Modify</Button><Button type="dashed">Flag</Button></div></div></div></div>
            //   lastIdx = {idx === conversations.length - 1}

           ))} 
           {loading ? <Flex align="center" gap="middle">
                      <Spin size="small" />
                      <Spin />
                      <Spin size="large" />
                      </Flex> : null}

    </div>
    <div></div>
    </div>
    </div>
    )
 }
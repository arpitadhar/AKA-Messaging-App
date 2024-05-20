import * as React from "react";
import { Link, useNavigate } from "react-router-dom";
import { useEffect, useRef, useState } from "react";
import "./adminPanel.css"; 
import SidebarAdmin from "../SidebarAdmin/SidebarAdmin";
import adminpng from "./admin.png"; 
import { RiAdminFill } from "react-icons/ri";
//import { MDBCol, MDBContainer, MDBRow, MDBCard, MDBCardTitle, MDBCardText, MDBCardBody, MDBCardImage, MDBBtn } from 'mdb-react-ui-kit';
import { Menu } from "antd"; 
import { Button, Flex } from "antd"; 
//import { useNavigate } from "react-router-dom";
export default function AdminPanel(props) {
    //console.log(props.user);
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
        key: "/", 
     }, 
     {
        label: "Flagged", 
        key: "/Flagged", 
     }, 
      
        ]}></Menu>
    </div>
    <div class="pageContent"> 
    <div className="boxes"><div className="inside"><div className = "pfp"></div><div className = "user-name"> <div>John Doe</div><div>-johndoe@gmail.com</div><div className = "user-buttons"> <Button danger>Delete User</Button><br></br><div></div><Button type="primary">Modify</Button><Button type="dashed">Flag</Button></div></div></div></div>
    <div className="boxes"><div className="inside"><div className = "pfp"></div><div className = "user-name"> <div>John Doe</div><div>-johndoe@gmail.com</div><div className = "user-buttons"> <Button danger>Delete User</Button><br></br><div></div><Button type="primary">Modify</Button><Button type="dashed">Flag</Button></div></div></div></div>
    <div className="boxes"><div className="inside"><div className = "pfp"></div><div className = "user-name"> <div>John Doe</div><div>-johndoe@gmail.com</div><div className = "user-buttons"> <Button danger>Delete User</Button><br></br><div></div><Button type="primary">Modify</Button><Button type="dashed">Flag</Button></div></div></div></div>
    </div>
    <div></div>
    </div>
    </div>
    )
 }
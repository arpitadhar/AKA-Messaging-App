import * as React from "react"
import { useState, useEffect } from "react"
import { Link , Navigate, useNavigate} from "react-router-dom"
import "./Navbar.css"
import { DownOutlined, SmileOutlined } from '@ant-design/icons';
import { Dropdown, Space } from 'antd';
 

// const navigate = useNavigate();


export default function Navbar({ isLoggedIn, handleLogout}) {
   const user1 = localStorage.getItem("email"); 
   //const user = user1.replace(/^"(.*)"$/, '$1');
   const items = [
    {
      key: '1',
      label: (
        <a target="_blank" rel="noopener noreferrer" href="/user"
        >
          Profile
        </a>
      ),
    },
    {
      key: '2',
      danger: true,
      label: 'Flag a User',
    },
  ];
    return (
        <nav className="navbar navbar-light bg-red" id="navbar" style={{ fontFamily: 'Rubik, sans-serif',  backgroundColor: 'rgba(255, 255, 255, 0.5)' }}>
            <a className="navbar-brand text-black px-3" href="/">AKA Messaging</a>
            {isLoggedIn ? (
                <div class="px-3">
                   <Dropdown
                        menu={{
                            items,
                        }}
                    >
                    <a onClick={(e) => e.preventDefault()}>
                    <Space>
                    {localStorage.getItem("email")}
                    <DownOutlined />
                    </Space>
                    </a>
                    </Dropdown>
                    <button onClick={handleLogout}>LogOut</button>

                </div>
            ) : (
                <div>
                  Create an account to start messaging!
                </div>
            )}

        </nav>
    )
}
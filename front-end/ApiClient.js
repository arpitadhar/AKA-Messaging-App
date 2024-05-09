import axios from "axios";

class ApiClient {
  constructor(remoteHostUrl) {
    this.remoteHostUrl = remoteHostUrl;
    this.token = null;
    this.tokenName = "ff_token";
  }

  //sets the token
  setToken(token) {
    this.token = token;
    localStorage.setItem(this.tokenName, token);
  }

  //Removes the token from storage.
  removeToken() {
    this.token = null;
    localStorage.removeItem(this.tokenName);
    return;
  }

  //Issues axios requests
  async request({ endpoint, method = "GET", data = {} }) {
    const url = `${this.remoteHostUrl}/${endpoint}`;
    const headers = {
      "Content-Type": "application/json",
    };
    if (this.token) {
      headers["Authorization"] = `Bearer ${this.token}`;
    }
    try {
      const request = await axios({ url, method, data, headers });
      if (request) {
        return { data: request.data, error: null };
      }
    } catch (err) {
      return { data: null, error: err };
    }
  }

//File Images Requests
async imageRequest({ endpoint, method = "POST", data = {} }) {
  //console.log(data)
  const url = `${this.remoteHostUrl}/${endpoint}`;
  let formData = new FormData();
  formData.set("file", data.image);
  const headers = {
    "Content-Type": "multipart/form-data",
  };
  if (this.token) {
    headers["Authorization"] = `Bearer ${this.token}`;
  }
  headers["ProfileId"] = data.profile_id;
  try {
    const request = await axios.post(url, formData, { headers: headers });
    if (request) {
      return request.data.addImage;
    }
  } catch (err) {
    return { data: null, error: err };
  }
}

  //Authentication Endpoints
  //Sends data to be register and returns the response.
  async register(data) {
    return await this.request({
      endpoint: "auth/register",
      method: "POST",
      data: data,
    });
  }
  //Sends data to log user and returns the response.
  async login(data) {
    return await this.request({
      endpoint: "auth/login",
      method: "POST",
      data: data,
    });
  }
  //Returns user from the token.
  async fetchUserFromToken() {
    return await this.request({ endpoint: "auth/me", method: "GET" });
  }


  //---------------------------//
  //Admin Endpoints

  //Returns all flagged users
  async getFlaggedUsers() {
    return await this.request({ endpoint: "admin/users", method: "GET" });
  }

  //Deletes post from site

  //Deletes user from site
  async deleteUser(user_id) {
    return await this.request({
      endpoint: "admin/deleteUser",
      method: "POST",
      data: {id: user_id},
    });
  }

  //Calls for a post to be unflagged, used when a admin deems a post appropriate.
  async unFlagPost(post_id, user_id){
    return await this.request({
      endpoint: `admin/unflag`,
      method: "POST",
      data: {post: post_id, user: user_id}
    })
  }


  //-----------------------//
  //Post Endpoints
  //Sends request to create post and sends another request to store image in that post
  //Cant send over image files using json content type, hence the reason for 2 requests.
  async uploadImage(data) {
    const addImage = await this.imageRequest({
      endpoint: "profile/upload",
      method: "POST",
      data: {image:data.image},
    });
    return addImage ;
  }

  async getImage(data) {
    const addImage = await this.imageRequest({
      endpoint: "profile/upload",
      method: "GET",
      data: {image:data.image},
    });
    return returnImage ;
  }

  //Send request to update user email
  async updateEmail(data) {
    return await this.request({
      endpoint:"profile/updateEmail", 
      method: "POST", 
      data:data })
  }

  async updatePassword(data) {
    return await this.request({
      endpoint:"profile/updatePassword", 
      method: "POST", 
      data:data })
  }

}

export default new ApiClient("http://0.0.0.0:3000/");
//();

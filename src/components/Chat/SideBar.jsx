import Conversations from "./conversations";
import SearchInput from "./searchInput"; 
const Sidebar = () => {

    return(
        <div className='border-r'> 
            <SearchInput/>
            <div className = 'divider3'></div>
            <Conversations/>
        </div>
    )
}

export default Sidebar
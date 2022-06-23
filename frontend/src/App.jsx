import { useState } from 'react'
import logo from './logo.svg'
import Box from '@mui/material/Box';
import Card from '@mui/material/Card';
import { FixedSizeList as List } from "react-window";
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import AutoSizer from "react-virtualized-auto-sizer";
import './App.css'
import { useEffect } from 'react';

function App() {
  const [packages, setPackages] = useState([])
  const [selectedIndex, setSelectedIndex] = useState(0);

  useEffect(() => {
    // GET request using fetch inside useEffect React hook
    fetch('http://localhost:3030/hello/test')
        .then(response => response.json())
        .then(data => {
          console.log("got the data")
          setPackages(data.packages)
        });

  // empty dependency array means this effect will only run once (like componentDidMount in classes)
  }, []);

  const row = (props) => {
    
    const { index, style } = props;
    const pkg = packages[index]
    return (
      <ListItem key={index} style={style} disablePadding>
        <ListItemButton selected={selectedIndex === index} onClick={() => { setSelectedIndex(index) }}>
          {pkg.name}
        </ListItemButton>
      </ListItem>
    )
  }

  const packageInfo = (pkg) => {

    return <div>
      <Card>{pkg.name}</Card>
      <Card>{pkg.arch}</Card>
      <Card>{pkg.desc}</Card>
      {JSON.stringify(pkg)}
    </div>
  }

  return (
    <div className="App">
        <Box sx={{ width: "100vw", height: "50vh", bgcolor: 'background.paper' }}>
          <AutoSizer>
            {({height, width}) => (
              <List
                className="List"
                height={height}
                width={width}
                itemCount={packages.length}
                itemSize={30}
                overscanCount={10}>
                  {row}
              </List>
            )}
          </AutoSizer>
        </Box>
        <Box sx={{ width: "100vw", height: "50vh", bgcolor: 'background.paper' }}>
            {
              (packages.length > 0) ? packageInfo(packages[selectedIndex]) : <></>
            }
        </Box>
    </div>
  )
}

export default App

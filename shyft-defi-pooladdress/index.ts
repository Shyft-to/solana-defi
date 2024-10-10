export async function getData() {
   const url = 'https://defi.shyft.to/';
   const response = await fetch(url);
     const data = await response.json();
     //console.log(data
    console.log(data);
}
getData()
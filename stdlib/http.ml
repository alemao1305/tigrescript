// Cliente HTTP simples
type Response = {
    status: int,
    body: string,
    headers: List<(string, string)>
}

type Request = {
    method: string,
    url: string,
    headers: List<(string, string)>,
    body: Option<string>
}

@extern("libcurl", "curl_easy_init")
native func curl_init() -> *mut void

@extern("libcurl", "curl_easy_setopt")
native func curl_setopt(handle: *mut void, option: int, value: *mut void) -> int

@extern("libcurl", "curl_easy_perform")
native func curl_perform(handle: *mut void) -> int

func fetch(url: string) -> Response {
    let handle = unsafe { curl_init() };
    
    unsafe {
        curl_setopt(handle, 10002 /* CURLOPT_URL */, url.as_ptr());
    }
    
    let mut response = String::new();
    
    // Configura callback para receber dados
    @extern("libcurl", "curl_easy_setopt")
    native func setopt_callback(handle: *mut void, option: int, callback: *mut void) -> int;
    
    unsafe {
        setopt_callback(handle, 20011 /* CURLOPT_WRITEFUNCTION */, write_callback as *mut void);
        setopt_callback(handle, 10001 /* CURLOPT_WRITEDATA */, &mut response as *mut _);
        
        let res = curl_perform(handle);
        
        if res != 0 {
            panic("Falha na requisição HTTP");
        }
    }
    
    Response {
        status: 200,
        body: response,
        headers: List::new()
    }
}

// Callback para escrita de dados
extern "C" fn write_callback(data: *mut u8, size: usize, nmemb: usize, userdata: *mut String) -> usize {
    unsafe {
        let slice = std::slice::from_raw_parts(data, size * nmemb);
        (*userdata).push_str(std::str::from_utf8(slice).unwrap());
    }
    size * nmemb
}
from selenium import webdriver
from selenium.webdriver.chrome.service import Service as ChromeService
from selenium.webdriver.firefox.service import Service as FirefoxService
from selenium.webdriver.edge.service import Service as EdgeService
from selenium.webdriver.chrome.options import Options as ChromeOptions
from selenium.webdriver.firefox.options import Options as FirefoxOptions
from selenium.webdriver.edge.options import Options as EdgeOptions
from webdriver_manager.chrome import ChromeDriverManager
from webdriver_manager.firefox import GeckoDriverManager
from webdriver_manager.microsoft import EdgeChromiumDriverManager
from selenium.webdriver.common.by import By
import time

def create_driver(browser):
    if browser == 'chrome':
        options = ChromeOptions()
        options.add_argument("--start-minimized")  # 启动时最小化
        service = ChromeService(ChromeDriverManager().install())
        return webdriver.Chrome(service=service, options=options)

    elif browser == 'firefox':
        options = FirefoxOptions()
        options.add_argument("--start-minimized")  # 启动时最小化
        service = FirefoxService(GeckoDriverManager().install())
        return webdriver.Firefox(service=service, options=options)

    elif browser == 'edge':
        options = EdgeOptions()
        options.add_argument("--start-minimized")  # 启动时最小化
        service = EdgeService(EdgeChromiumDriverManager().install())
        return webdriver.Edge(service=service, options=options)

    else:
        raise ValueError("Unsupported browser. Choose from 'chrome', 'firefox', or 'edge'.")

def get_cookies_after_login(url):
    browsers = ['firefox', 'edge', 'chrome']
    driver = None

    for browser in browsers:
        try:
            driver = create_driver(browser)
            driver.get(url)
            time.sleep(5)  # 等待页面加载

            # 检查是否存在登录按钮
            login_button_present = driver.find_elements(By.CSS_SELECTOR, '.right-entry__outside.go-login-btn')
            
            if login_button_present:
                # 点击登录按钮
                login_button_present[0].click()
                time.sleep(1)  # 等待页面加载

                # 获取用户输入
                username = input("请输入用户名: ")
                password = input("请输入密码: ")

                # 填写用户名和密码
                username_field = driver.find_element(By.CSS_SELECTOR, 'input[placeholder="请输入账号"]')
                password_field = driver.find_element(By.CSS_SELECTOR, 'input[placeholder="请输入密码"]')
                login_submit_button = driver.find_element(By.CSS_SELECTOR, '.btn_primary')
                
                username_field.send_keys(username)
                password_field.send_keys(password)
                login_submit_button.click()

                # 提示用户完成验证码验证
                print("请完成验证码验证后，按回车键继续...")
                input("按回车键继续...")
                
                time.sleep(5)  # 等待登录完成

            # 获取 cookies
            cookies = driver.get_cookies()
            return cookies
        
        except Exception as e:
            print(f"{browser} 浏览器无法启动或执行操作: {e}")
            if driver:
                driver.quit()
    
    raise RuntimeError("无法启动任何浏览器。")

# 使用示例
url = "https://www.bilibili.com/"  # Bilibili 登录页面 URL

cookies = get_cookies_after_login(url)
print("获取到的 Cookies:")
for cookie in cookies:
    print(cookie)

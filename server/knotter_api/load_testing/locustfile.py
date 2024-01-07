from locust import HttpUser, task

class HelloWorldUser(HttpUser):
    @task
    def hello_world(self):
        self.client.get("/health")
        self.client.get("/gvtest123/0")
        self.client.get("/gvtest123/1703018725690920482")
        self.client.get("/gvtest123/1703019365846429131")
        self.client.get("/gvtest123/1703090860610908919")
        self.client.get("/gvtest123/1703107258813712473")
        self.client.get("/gvtest123/1703107258813712473")
        self.client.get("/hjerte/0")
        self.client.get("/hjerte/1703020154617762028")
        self.client.get("/hjerte/1703083098460936447")
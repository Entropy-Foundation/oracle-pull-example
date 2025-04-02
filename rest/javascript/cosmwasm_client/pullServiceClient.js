const axios = require('axios');

class PullServiceClient {
  constructor(baseURL) {
    this.client = axios.create({
      baseURL: baseURL,
    });
  }

  async getProof(request) {
    try {
      const response = await this.client.post('/get_proof', request);
      return response.data;
    } catch (error) {
      throw error;
    }
  }
}

module.exports = PullServiceClient;
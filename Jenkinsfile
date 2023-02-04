pipeline {
  agent {
    docker {
      image 'rust-armv7'
    }

  }
  stages {
    stage('Build') {
      steps {
        sh 'cargo build --release'
      }
    }

  }
}
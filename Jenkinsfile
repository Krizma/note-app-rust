pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        echo "Setting up cargo"
        sh "source $HOME/.cargo/env"
        sh "export PATH=$HOME/.cargo/bind:$PATH"
        echo "Building..."
        sh 'cargo build'
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

  }
}
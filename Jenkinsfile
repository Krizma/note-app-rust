pipeline {
  agent any
  stages {
    stage('Build') {
      steps {
        echo "Getting Rust"
        sh """
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
        export PATH=$HOME/.cargo/bind:$PATH
        rustc --version
        """
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